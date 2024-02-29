package me.underworld.dante.custom_settings;

import android.content.Context;
import android.content.SharedPreferences;
import android.content.pm.PackageManager;
import android.view.View;
import android.widget.TextView;

import com.hotmail.or_dvir.easysettings.enums.ESettingsTypes;
import com.hotmail.or_dvir.easysettings.pojos.SettingsObject;

import java.io.Serializable;

import me.underworld.dante.DanteGlobal;
import me.underworld.dante.R;
import me.underworld.dante.helpers.LangDict;

public class TextSettingsObject extends SettingsObject<Void> implements Serializable {

    public TextSettingsObject(Builder builder) {
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
        tv.setText(this.getTitle());
    }

    public static class Builder extends SettingsObject.Builder<Builder,Void> {
        public View.OnClickListener listener;
        public Builder(String key, String title, View.OnClickListener listener) {
            super(key, title, null, 0, null, ESettingsTypes.VOID, null);
        }

        @Override
        public TextSettingsObject build() {
            return new TextSettingsObject(this);
        }
    }
}
