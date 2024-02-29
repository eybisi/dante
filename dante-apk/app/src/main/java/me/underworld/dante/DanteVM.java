package me.underworld.dante;


import android.app.Activity;
import android.app.Application;
import android.content.Context;
import android.content.Intent;
import android.graphics.drawable.Drawable;
import android.os.Build;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.FrameLayout;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.core.app.ActivityCompat;
import androidx.core.content.ContextCompat;

import com.imuxuan.floatingview.FloatingMagnetView;
import com.imuxuan.floatingview.MagnetViewListener;

import java.io.File;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;

import dalvik.system.BaseDexClassLoader;
import dalvik.system.DexClassLoader;
import dalvik.system.PathClassLoader;
import me.underworld.dante.helpers.FG;
import me.underworld.dante.views.FloatingIcon;
import me.underworld.dante.views.SettingsView;
import me.underworld.dante.views.TransparentView;


// HLVM
public class DanteVM implements MagnetViewListener, TransparentView.TransparentClickListener {
    public static DanteVM instance = null;
    public static Activity activity = null;
    public static FrameLayout rootElement = null;
    public static List<Runnable> runnableList = new ArrayList<>();

    public FloatingIcon floating_icon = new FloatingIcon();
    public SettingsView settingsView = new SettingsView();
//    public AboutView aboutView = new AboutView();

    public TransparentView transparentView = new TransparentView();

    @Override
    public void onMagnetViewRemoved(FloatingMagnetView magnetView) {

    }

    @Override
    public void onMagnetViewClicked(FloatingMagnetView magnetView) {
        if(magnetView.getTag().equals("icon")){
            if(transparentView.visible){
               transparentView.setViewGone();
            }else{
                transparentView.setViewVisible();
//                float viewX = ((magnetView.getX() - transparentView.root.getWidth()) - 5.0f) + magnetView.getWidth();
                float viewX = magnetView.getX() + 5.0f;
                transparentView.setX(viewX);
                float viewY = magnetView.getY() + magnetView.getHeight() + 5.0f;
                transparentView.setY(viewY);
            }
        }
    }

    @Override
    public void onMangetViewMoved(FloatingMagnetView magnetView, float x, float y) {
        if(magnetView.getTag().equals("icon")){
            if(transparentView.visible){
//                float viewX = ((magnetView.getX() - transparentView.root.getWidth()) - 5.0f) + magnetView.getWidth();
                float viewX = magnetView.getX() + 5.0f;
                transparentView.setX(viewX);
                float viewY = magnetView.getY() + magnetView.getHeight() + 5.0f;
                transparentView.setY(viewY);
            }
        }
    }

    public static synchronized DanteVM getInstance() {
        DanteVM danteVM;
        synchronized (DanteVM.class) {
            if (instance == null) {
                instance = new DanteVM();
            }
            danteVM = instance;
        }
        return danteVM;
    }

    public void initActivity(Activity act) {
        synchronized (DanteVM.class) {
            activity = act;
            for(Runnable runnable : runnableList) {
                activity.runOnUiThread(runnable);
            }
            runnableList.clear();
        }
        rootElement = getRootElement(act);
        initializeViews();
    }

    public void initializeViews(){
        floating_icon.init(activity, rootElement);
        floating_icon.floating_view.setMagnetViewListener(this);
        floating_icon.addViewsToWindow();
        floating_icon.setViewVisible();

        settingsView.init(activity, rootElement);
        settingsView.addViewsToWindow();
        settingsView.setViewGone();

        transparentView.init(activity, rootElement);
        transparentView.transparentClickListener = this;
        transparentView.addViewsToWindow();
        transparentView.setViewGone();
    }

    public static FrameLayout getRootElement(Activity activity) {
        try {
            return (FrameLayout) activity.getWindow().getDecorView().findViewById(android.R.id.content);
        } catch (Exception e2) {
            e2.printStackTrace();
            return null;
        }
    }

    @Override
    public void TransparentItemOnClick(String tag, View view) {
        if(tag.equals("settings")){
            transparentView.setViewGone();
            settingsView.changeState();
        }else if(tag.equals("about")){
            // Show toast message
            Toast.makeText(DanteVM.activity,"Hellbot v1.1",Toast.LENGTH_SHORT).show();
        }

    }

    public static class DanteLifecycle implements Application.ActivityLifecycleCallbacks {

        public Activity lifecycle_activity = null;
        public class UiThread extends Thread {
            public Activity activity = null;
            public FrameLayout rootElement = null;

            public UiThread(Activity activity) {
                this.activity = activity;
            }


            @Override
            public void run () {
                rootElement = getRootElementOfActivity();
                while (rootElement.getHeight() == 0) {
                    try {
                        Thread.sleep(1000L);
                    } catch (InterruptedException unused) {
                    }
                }
                activity.runOnUiThread(new Runnable() {
                    @Override
                    public void run() {
                        DanteVM.getInstance().initActivity(activity);
                    }
                });

            }
            public FrameLayout getRootElementOfActivity() {
                return (FrameLayout) this.activity.findViewById(android.R.id.content);
            }
        }


        @Override
        public void onActivityCreated(android.app.Activity activity, android.os.Bundle savedInstanceState) {

        }

        @Override
        public void onActivityStarted(@NonNull Activity activity) {

        }

        @Override
        public void onActivityResumed(@NonNull Activity activity) {
            Log.d("DanteLifecycle", "Activity resumed");
            if(this.lifecycle_activity == activity) {
                return;
            }else{
                this.lifecycle_activity = activity;
            }
            UiThread uiThread = new UiThread(activity);
            uiThread.start();
        }

        @Override
        public void onActivityPaused(@NonNull Activity activity) {

        }

        @Override
        public void onActivityStopped(@NonNull Activity activity) {
        }

        @Override
        public void onActivitySaveInstanceState(@NonNull Activity activity, @NonNull Bundle bundle) {

        }

        @Override
        public void onActivityDestroyed(@NonNull Activity activity) {
            this.lifecycle_activity = null;


        }


    }
}


