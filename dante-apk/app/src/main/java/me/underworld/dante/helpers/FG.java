package me.underworld.dante.helpers;

import android.app.Notification;
import android.app.Service;
import android.content.Intent;
import android.os.IBinder;
import android.util.Log;

import androidx.annotation.Nullable;
import androidx.core.app.NotificationCompat;

import me.underworld.dante.R;

public class FG extends Service {

    private static final int NOTIFICATION_ID = 123;
    private static final String CHANNEL_ID = "hela_plugin";

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        // Start your background processing here
        Log.d("FG", "Service started");
        // Create a notification for foreground service
        Notification notification = createNotification();
        startForeground(NOTIFICATION_ID, notification);

        return START_STICKY; // Service will be restarted if it gets killed by the system
    }

    @Nullable
    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    private Notification createNotification() {
        // Build your notification here
        // Example:
        NotificationCompat.Builder builder = new NotificationCompat.Builder(this, CHANNEL_ID)
                .setContentTitle("Foreground Service")
                .setContentText("Processing data...")
                .setSmallIcon(R.drawable.ic_notification)
                .setPriority(NotificationCompat.PRIORITY_DEFAULT);
        Log.d("FG", "Notification created");
        return builder.build();

    }
    @Override
    public void onDestroy() {
        super.onDestroy();
        stopForeground(true); // Stop foreground service and remove the notification
    }
}