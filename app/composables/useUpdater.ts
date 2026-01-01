import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export const useUpdater = () => {
  const toast = useToast();
  const availableUpdate = ref<Update | null>(null);
  const isDownloading = ref(false);
  const downloadProgress = ref(0);

  const checkForUpdates = async (): Promise<Update | null> => {
    try {
      const update = await check();
      console.log("Update check result:", update);

      if (update) {
        console.log(
          `Found update ${update.version} from ${update.date} with notes ${update.body}`
        );

        // Show toast notification with action button
        toast.add({
          title: "Update Available",
          description: `Version ${update.version} is available. Your current version is ${update.currentVersion}.`,
          color: "primary",
          icon: "i-lucide-download",
          progress: false,
          duration: Infinity, // Disable auto close
          actions: [
            {
              label: "Update Now",
              color: "primary",
              variant: "solid",
              onClick: () => {
                downloadAndInstallUpdate(update);
              },
            },
            {
              label: "Later",
              color: "neutral",
              variant: "ghost",
            },
          ],
        });

        // Store the update so UI can access it
        availableUpdate.value = update;
        return update;
      } else {
        console.log("No updates available");
        return null;
      }
    } catch (error) {
      console.error("Error checking for updates:", error);
      return null;
    }
  };

  const downloadAndInstallUpdate = async (
    update: Update | null
  ): Promise<void> => {
    if (!update) {
      console.error("No update provided");
      return;
    }
    if (isDownloading.value) {
      console.log("Update already in progress");
      return;
    }

    isDownloading.value = true;
    downloadProgress.value = 0;

    let downloadToast: ReturnType<typeof toast.add> | undefined;

    try {
      let downloaded = 0;
      let contentLength = 0;

      // Show download started notification with progress
      downloadToast = toast.add({
        title: "Downloading Update",
        description: `Downloading version ${update.version}... 0%`,
        color: "primary",
        icon: "i-lucide-download",
        duration: Infinity, // Disable auto close
      });

      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength || 0;
            console.log(
              `Started downloading ${event.data.contentLength} bytes`
            );
            downloadProgress.value = 0;
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            const percent =
              contentLength > 0
                ? Math.round((downloaded / contentLength) * 100)
                : 0;
            downloadProgress.value = percent;
            console.log(
              `Downloaded ${downloaded} from ${contentLength} (${percent}%)`
            );

            // Update progress in toast (safe - won't affect download if toast is closed)
            if (downloadToast) {
              try {
                toast.update(downloadToast.id, {
                  description: `Downloading version ${update.version}... ${percent}%`,
                });
              } catch (error) {
                // Toast was closed by user, continue download in background
                downloadToast = undefined;
              }
            }
            break;
          case "Finished":
            console.log("Download finished");
            downloadProgress.value = 100;
            if (downloadToast) {
              try {
                toast.update(downloadToast.id, {
                  description: `Download complete. Installing...`,
                });
              } catch (error) {
                // Toast was closed by user, continue in background
                downloadToast = undefined;
              }
            }
            break;
        }
      });

      console.log("Update installed");

      // Remove download toast if it still exists (safe - won't affect if already closed)
      if (downloadToast) {
        try {
          toast.remove(downloadToast.id);
        } catch (error) {
          // Toast was already closed, ignore
        }
      }

      // Show toast notification
      toast.add({
        title: "Update Installed",
        description:
          "The update has been installed. The app will restart shortly.",
        color: "success",
        icon: "i-lucide-check-circle",
        duration: Infinity, // Disable auto close
      });

      // Small delay to ensure notification is shown
      await new Promise((resolve) => setTimeout(resolve, 2000));

      await relaunch();
    } catch (error) {
      console.error("Error downloading/installing update:", error);
      toast.add({
        title: "Update Failed",
        description:
          "Failed to download or install the update. Please try again later.",
        color: "error",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    } finally {
      isDownloading.value = false;
    }
  };

  const installUpdate = async (): Promise<void> => {
    const update = availableUpdate.value;
    if (update) {
      await downloadAndInstallUpdate(update as Update);
    }
  };

  return {
    checkForUpdates,
    downloadAndInstallUpdate,
    installUpdate,
    availableUpdate: readonly(availableUpdate),
    isDownloading: readonly(isDownloading),
  };
};
