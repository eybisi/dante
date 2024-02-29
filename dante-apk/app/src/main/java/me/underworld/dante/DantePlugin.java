package me.underworld.dante;

import android.app.Application;
import android.content.res.AssetManager;
import android.content.res.Resources;
import android.os.Build;
import android.util.Log;

import androidx.annotation.Keep;

import com.hotmail.or_dvir.easysettings.pojos.EasySettings;
import com.hotmail.or_dvir.easysettings.pojos.SettingsObject;
import com.hotmail.or_dvir.easysettings.pojos.SwitchSettingsObject;

import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.ArrayList;

import me.underworld.dante.custom_settings.TextSettingsObject;
import me.underworld.dante.custom_settings.VersionSettingsObject;
import me.underworld.dante.helpers.DanteBotSettings;
import me.underworld.dante.helpers.LangDict;

// PL
public class DantePlugin {

    public static Resources resources;
    public static ArrayList<SettingsObject> settingsObjects = new ArrayList<>();

    @Keep
    public static native void pushSettings(int settingIndex, Object obj);



    // This method needs be called from the native code
    // PluginPath is where this plugin is located (after decrypted)
    @Keep
    public static void initNative(Application application, String pluginPath) {

        Log.d("DantePlugin", "Apk is being initialized");
        DanteGlobal.setCtx(application.getApplicationContext());
        DanteGlobal.setApplication(application);
        // Create new pathclass loader with plugin path
//        DanteGlobal.classLoader = new PathClassLoader(pluginPath, application.getApplicationInfo().nativeLibraryDir, ClassLoader.getSystemClassLoader());
        addLoadedApkResource(application, pluginPath);
        LangDict.loadStringsFromFile("tr");
        addSettingsObject();
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.ICE_CREAM_SANDWICH) {
            application.registerActivityLifecycleCallbacks(new DanteVM.DanteLifecycle());
            application.getCacheDir();
        }
        // Initialize settings
        Log.d("DantePlugin", "Apk is initialized");
    }
    public static void addLoadedApkResource(Application application, String resourceName)  {
        try{
            AssetManager am = AssetManager.class.newInstance();
            Method m = AssetManager.class.getMethod("addAssetPath", String.class);
            m.invoke(am,application.getBaseContext().getPackageResourcePath());
            m.invoke(am, resourceName);
            Resources r = new Resources(am, application.getBaseContext().getResources().getDisplayMetrics(), application.getBaseContext().getResources().getConfiguration());
            Field field = application.getBaseContext().getClass().getDeclaredField("mResources");
            field.setAccessible(true);
            field.set(application.getBaseContext(), r);
            Field field2 = application.getBaseContext().getClass().getDeclaredField("mPackageInfo");
            field2.setAccessible(true);
            Object obj = field2.get(application.getBaseContext());
            Field declaredField3 = obj.getClass().getDeclaredField("mResources");
            declaredField3.setAccessible(true);
            declaredField3.set(obj, r);
            resources = r;
            DanteGlobal.resources = resources;

        } catch (IllegalAccessException | InstantiationException | NoSuchMethodException | InvocationTargetException e) {
            Log.d("DantePlugin", "addLoadedApkResource error: " + e.getMessage());
        } catch (NoSuchFieldException e) {
            Log.d("DantePlugin", "addLoadedApkResource error: " + e.getMessage());
            throw new RuntimeException(e);
        }
    }

    public static void addSettingsObject() {

        settingsObjects = EasySettings.createSettingsArray(


                new SwitchSettingsObject.Builder("dt_lup", LangDict.getStringValue("dt_lup"), true)
                        .addDivider()
                        .setListener(new SettingsObject.SettingsListener<Boolean>() {
                            @Override
                            public void onSettingsChanged(SettingsObject settingsObject, Boolean newValue, Boolean oldValue) {
                                if(newValue != null){
                                    DantePlugin.pushSettings(DanteBotSettings.LUPON_HACK.value, newValue);
                                }
                            }
                        })
                        .build(),
                new SwitchSettingsObject.Builder("dt_genie", LangDict.getStringValue("dt_genie"), true)
                        .addDivider()
                        .setListener(new SettingsObject.SettingsListener<Boolean>() {
                            @Override
                            public void onSettingsChanged(SettingsObject settingsObject, Boolean newValue, Boolean oldValue) {
                                if(newValue != null){
                                    DantePlugin.pushSettings(DanteBotSettings.INFINITE_GENIE.value, newValue);
                                }
                            }
                        })
                        .build(),
                new SwitchSettingsObject.Builder("dt_captcha_bypass", LangDict.getStringValue("dt_captcha_bypass"), true)
                        .addDivider()
                        .setListener(new SettingsObject.SettingsListener<Boolean>() {
                            @Override
                            public void onSettingsChanged(SettingsObject settingsObject, Boolean newValue, Boolean oldValue) {
                                if (newValue != null) {
                                DantePlugin.pushSettings(DanteBotSettings.IMNOTROBOT_BYPASS.value, newValue);
                                }
                            }
                        })
                        .build(),
                new SwitchSettingsObject.Builder("dt_move_in_skill_animation", LangDict.getStringValue("dt_move_in_skill_animation"), true)
                        .addDivider()
                        .setListener(new SettingsObject.SettingsListener<Boolean>() {
                            @Override
                            public void onSettingsChanged(SettingsObject settingsObject, Boolean newValue, Boolean oldValue) {
                                if(newValue != null) {

                                    DantePlugin.pushSettings(DanteBotSettings.MOVE_IN_ANIMATION.value, newValue);
                                }
                            }
                        })
                        .build(),
                new SwitchSettingsObject.Builder("dt_auto_loot", LangDict.getStringValue("dt_auto_loot"), true)
                        .addDivider()
                        .setListener(new SettingsObject.SettingsListener<Boolean>() {
                            @Override
                            public void onSettingsChanged(SettingsObject settingsObject, Boolean newValue, Boolean oldValue) {
                                if(newValue != null) {
                                    DantePlugin.pushSettings(DanteBotSettings.AUTO_LOOT.value, newValue);
                                }
                             }
                        })
                        .build(),
//                new ButtonSettingsObject.Builder("dt_button_test", LangDict.getStringValue("dt_button_test"), new View.OnClickListener() {
//                    @Override
//                    public void onClick(View v) {
////                        DantePlugin.pushSettings(DanteBotSettings.RESET.value, true);
//                        Toast.makeText(DanteGlobal.ctx, "Reset", Toast.LENGTH_SHORT).show();
//                    }
//                })
//                        .addDivider()
//                        .build(),
                new SwitchSettingsObject.Builder("dt_emulator", LangDict.getStringValue("dt_emulator"), true)
                        .addDivider()
                        .setListener(new SettingsObject.SettingsListener<Boolean>() {
                            @Override
                            public void onSettingsChanged(SettingsObject settingsObject, Boolean newValue, Boolean oldValue) {
                                if(newValue != null){
                                    DantePlugin.pushSettings(DanteBotSettings.EMULATOR_BYPASS.value, newValue);
                                }

                            }
                        })
                        .build(),
                new TextSettingsObject.Builder("dt_botname", LangDict.getStringValue("dt_botname"), null)
                        .addDivider()
                        .build(),
                new VersionSettingsObject.Builder("dt_version", null)
                        .build()
                );


        EasySettings.initializeSettings(DanteGlobal.ctx, settingsObjects);
    }
}
