package me.underworld.dante.views;

import android.content.SharedPreferences;
import android.content.res.Resources;
import android.graphics.Color;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

import com.hotmail.or_dvir.easysettings.pojos.EasySettings;
import com.hotmail.or_dvir.easysettings.pojos.SettingsObject;

import java.util.ArrayList;

import me.underworld.dante.DantePlugin;
import me.underworld.dante.R;

public class SettingsView extends DanteViewGroup {

    public void hideView(View view) {
        setViewGone();
    }

    @Override
    public void addViews() {
        SharedPreferences settingsSharedPrefs = EasySettings.retrieveSettingsSharedPrefs(ctx);
        ArrayList<SettingsObject> mySettingsList = DantePlugin.settingsObjects;
        LinearLayout root = (LinearLayout) inflateWithCtx(R.layout.dt_settings, ctx);
        root.setBackgroundColor(Color.WHITE);
        LinearLayout newLayout = new LinearLayout(ctx);
        LinearLayout.LayoutParams lp = new LinearLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, -2);
        Resources resource = resources;
        lp.setMargins(0, 15, 0,  15);
        newLayout.setLayoutParams(lp);
        newLayout.setOrientation(LinearLayout.VERTICAL);
        int dimensionPixelSize = resource.getDimensionPixelSize(me.underworld.dante.R.dimen.settings_container_padding);
        newLayout.setPadding(dimensionPixelSize,0,dimensionPixelSize,dimensionPixelSize);
        ScrollView sv = new ScrollView(ctx);
        sv.setLayoutParams(new ViewGroup.LayoutParams(-1, -2));
        sv.addView(newLayout);
        root.addView(sv);
        EasySettings.inflateSettingsLayout(ctx, newLayout, mySettingsList);
        Views.add(root);
        TextView textView = (TextView) root.findViewById(R.id.dt_st_close);
        textView.setTextSize(2, 18.0f);
        textView.setOnClickListener(new View.OnClickListener(){
            @Override
            public void onClick(View v) {
                hideView(v);
            }
        });
    }
}
