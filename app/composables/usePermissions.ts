import {
  checkInputMonitoringPermission,
  requestInputMonitoringPermission,
  checkAccessibilityPermission,
  requestAccessibilityPermission,
} from "tauri-plugin-macos-permissions-api";

export const usePermissions = () => {
  const isMacOS = async (): Promise<boolean> => {
    try {
      // Try to check permission - if it fails, we're likely not on macOS
      await checkInputMonitoringPermission();
      console.log("[Permissions] Detected macOS");
      return true;
    } catch (e) {
      console.log("[Permissions] Not macOS or plugin unavailable:", e);
      return false;
    }
  };

  const checkInputMonitoring = async (): Promise<boolean> => {
    try {
      const result = await checkInputMonitoringPermission();
      console.log("[Permissions] Input Monitoring check result:", result);
      return result;
    } catch (e) {
      // Not on macOS or plugin not available
      console.log(
        "[Permissions] Input Monitoring check failed (non-macOS?):",
        e
      );
      return true; // Return true on non-macOS to allow normal operation
    }
  };

  const requestInputMonitoring = async (): Promise<boolean> => {
    try {
      console.log("[Permissions] Requesting Input Monitoring permission...");
      const result = await requestInputMonitoringPermission();
      console.log("[Permissions] Input Monitoring request result:", result);
      return Boolean(result);
    } catch (error) {
      console.error(
        "[Permissions] Failed to request input monitoring permission:",
        error
      );
      return false;
    }
  };

  const checkAccessibility = async (): Promise<boolean> => {
    try {
      const result = await checkAccessibilityPermission();
      console.log("[Permissions] Accessibility check result:", result);
      return result;
    } catch (e) {
      // Not on macOS or plugin not available
      console.log("[Permissions] Accessibility check failed (non-macOS?):", e);
      return true; // Return true on non-macOS to allow normal operation
    }
  };

  const requestAccessibility = async (): Promise<boolean> => {
    try {
      console.log("[Permissions] Requesting Accessibility permission...");
      const result = await requestAccessibilityPermission();
      console.log("[Permissions] Accessibility request result:", result);
      return Boolean(result);
    } catch (error) {
      console.error(
        "[Permissions] Failed to request accessibility permission:",
        error
      );
      return false;
    }
  };

  const ensureInputMonitoringPermission = async (): Promise<boolean> => {
    const isMac = await isMacOS();
    if (!isMac) {
      return true; // Not macOS, no permission needed
    }

    const hasPermission = await checkInputMonitoring();
    if (hasPermission) {
      return true;
    }

    return await requestInputMonitoring();
  };

  const ensureAccessibilityPermission = async (): Promise<boolean> => {
    const isMac = await isMacOS();
    if (!isMac) {
      return true; // Not macOS, no permission needed
    }

    const hasPermission = await checkAccessibility();
    if (hasPermission) {
      return true;
    }

    return await requestAccessibility();
  };

  const ensureHIDPermissions = async (
    retryCount = 0
  ): Promise<{
    accessibility: boolean;
    inputMonitoring: boolean;
    allGranted: boolean;
  }> => {
    console.log(
      "[Permissions] ensureHIDPermissions called (attempt:",
      retryCount + 1,
      ")"
    );
    const isMac = await isMacOS();
    console.log("[Permissions] isMacOS result:", isMac);

    if (!isMac) {
      console.log("[Permissions] Not macOS, returning all granted");
      return {
        accessibility: true,
        inputMonitoring: true,
        allGranted: true,
      };
    }

    // Check both permissions (don't request, just check current status)
    // Run checks in parallel for faster response
    console.log("[Permissions] Starting parallel permission checks...");
    const [accessibilityGranted, inputMonitoringGranted] = await Promise.all([
      checkAccessibility(),
      checkInputMonitoring(),
    ]);

    const result = {
      accessibility: accessibilityGranted,
      inputMonitoring: inputMonitoringGranted,
      allGranted: accessibilityGranted && inputMonitoringGranted,
    };

    console.log("[Permissions] Final permission status:", result);

    // If permissions are false but this is the first attempt, retry once after a short delay
    // This handles cases where the plugin hasn't fully initialized yet
    if (!result.allGranted && retryCount === 0) {
      console.log(
        "[Permissions] Permissions false on first check, retrying after 200ms..."
      );
      await new Promise((resolve) => setTimeout(resolve, 200));
      return ensureHIDPermissions(1);
    }

    return result;
  };

  return {
    checkInputMonitoring,
    requestInputMonitoring,
    ensureInputMonitoringPermission,
    checkAccessibility,
    requestAccessibility,
    ensureAccessibilityPermission,
    ensureHIDPermissions,
    isMacOS,
  };
};
