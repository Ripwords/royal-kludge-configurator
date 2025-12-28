import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { sendNotification } from "@tauri-apps/plugin-notification";

export const useUpdater = () => {
  const checkForUpdates = async (): Promise<void> => {
    try {
      const update = await check();

      if (update) {
        console.log(
          `Found update ${update.version} from ${update.date} with notes ${update.body}`
        );

        await sendNotification({
          title: "Update Available",
          body: `Version ${update.version} is available. Downloading...`,
        });

        let downloaded = 0;
        let contentLength = 0;

        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case "Started":
              contentLength = event.data.contentLength || 0;
              console.log(
                `Started downloading ${event.data.contentLength} bytes`
              );
              break;
            case "Progress":
              downloaded += event.data.chunkLength;
              const percent =
                contentLength > 0
                  ? Math.round((downloaded / contentLength) * 100)
                  : 0;
              console.log(
                `Downloaded ${downloaded} from ${contentLength} (${percent}%)`
              );
              break;
            case "Finished":
              console.log("Download finished");
              break;
          }
        });

        console.log("Update installed");

        await sendNotification({
          title: "Update Installed",
          body: "The update has been installed. The app will restart shortly.",
        });

        // Small delay to ensure notification is shown
        await new Promise((resolve) => setTimeout(resolve, 1000));

        await relaunch();
      } else {
        console.log("No updates available");
      }
    } catch (error) {
      console.error("Error checking for updates:", error);
      // Silently fail - don't interrupt user experience
    }
  };

  return {
    checkForUpdates,
  };
};
