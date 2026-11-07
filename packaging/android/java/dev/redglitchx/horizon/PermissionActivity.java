package dev.redglitchx.horizon;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.provider.Settings;
import android.widget.Toast;

/**
 * Shim Activity that requests MANAGE_EXTERNAL_STORAGE on Android 11+
 * before forwarding to the NativeActivity game engine.
 *
 * Flow:
 *   1. User taps app icon → PermissionActivity launches.
 *   2. If storage permission already granted → start NativeActivity immediately.
 *   3. Otherwise → open system Settings for "All Files Access" and wait.
 *   4. On resume (user comes back from Settings) → check again.
 *      If granted → start NativeActivity. If not → show Toast, stay.
 */
public class PermissionActivity extends Activity {

    private static final int REQUEST_MANAGE_STORAGE = 1001;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        if (hasStoragePermission()) {
            launchGame();
        } else {
            requestStoragePermission();
        }
    }

    @Override
    protected void onResume() {
        super.onResume();
        // User might return from Settings; recheck.
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == REQUEST_MANAGE_STORAGE) {
            if (hasStoragePermission()) {
                launchGame();
            } else {
                Toast.makeText(this,
                    "Horizon needs 'All Files Access' to load game resources from /sdcard/games/horizon/.\n" +
                    "Please grant the permission and reopen the app.",
                    Toast.LENGTH_LONG).show();
                // Stay on this activity so user can try again
            }
        }
    }

    private boolean hasStoragePermission() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            return Environment.isExternalStorageManager();
        }
        // Pre-Android 11: legacy storage or requestLegacyExternalStorage covers us
        return true;
    }

    private void requestStoragePermission() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            try {
                Intent intent = new Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION);
                intent.setData(Uri.parse("package:" + getPackageName()));
                startActivityForResult(intent, REQUEST_MANAGE_STORAGE);
            } catch (Exception e) {
                // Fallback: open general storage management settings
                Intent intent = new Intent(Settings.ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION);
                startActivityForResult(intent, REQUEST_MANAGE_STORAGE);
            }
        } else {
            // On older Android, we don't need this special permission
            launchGame();
        }
    }

    private void launchGame() {
        Intent intent = new Intent();
        intent.setClassName(this, "android.app.NativeActivity");
        // Pass through any extras
        if (getIntent().getExtras() != null) {
            intent.putExtras(getIntent().getExtras());
        }
        startActivity(intent);
        finish(); // Close the permission activity
    }
}
