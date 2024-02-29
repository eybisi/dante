package me.underworld.dante.custom_settings;

import android.content.Context;
import android.content.SharedPreferences;
import android.content.pm.PackageManager;
import android.view.View;
import android.widget.TextView;

import me.underworld.dante.R;
import com.hotmail.or_dvir.easysettings.enums.ESettingsTypes;
import com.hotmail.or_dvir.easysettings.pojos.SettingsObject;

import java.io.Serializable;

import me.underworld.dante.DanteGlobal;
import me.underworld.dante.helpers.LangDict;

public class VersionSettingsObject extends SettingsObject<Void> implements Serializable {

    public VersionSettingsObject(Builder builder) {
        super(builder.getKey(),
                builder.getTitle(),
                builder.getDefaultValue(),
                builder.getSummary(),
                builder.getTextViewTitleId(),
                builder.getTextViewSummaryId(),
                builder.getUseValueAsSummary(),
                builder.hasDivider(),
                builder.getType(),
                builder.getImageViewIconId(),
                builder.getIconDrawableId(),
                builder.getIconDrawable(),
                null
        );
    }

    @Override
    public int getLayout() {
        return R.layout.text_settings_object;
    }

    @Override
    public Void checkDataValidity(Context context, SharedPreferences prefs) {
        return null;
    }

    @Override
    public String getValueHumanReadable() {
        return null;
    }

    @Override
    public void initializeViews(View root) {
        super.initializeViews(root);
        TextView tv = (TextView) root.findViewById(R.id.hl_sto_switch_summary);
        tv.setTextColor(DanteGlobal.resources.getColor(R.color.dt_text_default));
        String str = "unknown";
        try{
            str = DanteGlobal.ctx.getPackageManager().getPackageInfo(DanteGlobal.ctx.getPackageName(), 0).versionName;
        }catch (PackageManager.NameNotFoundException e) {
        }
        tv.setText(LangDict.getStringValue("dt_version") + "1.1(" + str+ ")");
    }

    public static class Builder extends SettingsObject.Builder<Builder,Void> {
        public View.OnClickListener listener;
        public Builder(String key, View.OnClickListener listener) {
            super(key, null, null, 0, null, ESettingsTypes.VOID, null);
        }

        @Override
        public VersionSettingsObject build() {
            return new VersionSettingsObject(this);
        }
    }
}
