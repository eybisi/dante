package me.underworld.dante.custom_settings;

import android.content.Context;
import android.content.SharedPreferences;
import android.util.Log;
import android.view.View;
import android.widget.Button;

import me.underworld.dante.R;
import com.hotmail.or_dvir.easysettings.enums.ESettingsTypes;
import com.hotmail.or_dvir.easysettings.pojos.SettingsObject;

import java.io.Serializable;

public class ButtonSettingsObject extends SettingsObject<Void> implements Serializable {
    public final View.OnClickListener button_listener;

    public ButtonSettingsObject(Builder builder, View.OnClickListener button_listener) {
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
        this.button_listener = button_listener;
    }

    @Override
    public int getLayout() {
        return R.layout.settings_object_singlebutton;
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
        Button button = (Button) root.findViewById(R.id.hl_sto_sb_btn);
        Log.d("ButtonSettingsObject", "initializeViews: " + button);
        button.setOnClickListener(button_listener);
        Log.d("ButtonSettingsObject", "initializeViews: " + this.getTitle());
        button.setText(this.getTitle());
    }

    public static class Builder extends SettingsObject.Builder<Builder,Void> {
        public View.OnClickListener listener;
        public Builder(String key, String title, View.OnClickListener listener) {
            super(key, title, null, 0, null, ESettingsTypes.VOID, null);
            this.listener = listener;
        }

        @Override
        public ButtonSettingsObject build() {
            return new ButtonSettingsObject(this, listener);
        }
    }
}
