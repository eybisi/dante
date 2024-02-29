package me.underworld.dante;

import android.app.Application;
import android.content.Context;
import android.content.SharedPreferences;
import android.content.res.Resources;

import dalvik.system.PathClassLoader;

// GL
public class DanteGlobal {
    public static Context ctx = null;
    public static Application application = null;
    public static SharedPreferences sharedPreferences = null;
    public static Resources resources = null;

    public static void setCtx(Context c) {
        ctx = c;
    }
    public static void setApplication(Application a) {
        application = a;
    }
    public static void setSharedPreferences(SharedPreferences s) {
        sharedPreferences = s;
    }
    public static void setResources(Resources r) {
        resources = r;
    }

}
