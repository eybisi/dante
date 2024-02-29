package me.underworld.dante;

public class MainApplication extends android.app.Application {

    @Override
    public void onCreate() {
        super.onCreate();
        DanteVM danteVM = DanteVM.getInstance();

        DantePlugin.initNative(this, getApplicationInfo().sourceDir);
    }

}
