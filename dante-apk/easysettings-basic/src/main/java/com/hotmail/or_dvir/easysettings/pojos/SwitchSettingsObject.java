package com.hotmail.or_dvir.easysettings.pojos;

import android.content.SharedPreferences;

import com.hotmail.or_dvir.easysettings.R;
import java.io.Serializable;

/**
 * a setting object which contains a {@link android.widget.Switch}
 */
@SuppressWarnings("PointlessBooleanExpression")
public class SwitchSettingsObject extends BooleanSettingsObject implements Serializable
{
	public SwitchSettingsObject(Builder builder)
	{
		super(builder);
	}

	@Override
	public int getLayout()
	{
		return R.layout.settings_object_switch;
	}

	/////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////
	/////////////////////////////////////////////////////////////

	public static class Builder extends BooleanSettingsObject.Builder
	{
		/**
		 *
		 * @param key the key for this {@link SwitchSettingsObject}
		 *            to be saved in the apps' {@link SharedPreferences}
		 * @param title the title for this {@link SwitchSettingsObject}
		 * @param defaultValue
		 */
        public Builder(String key,
                       String title,
                       boolean defaultValue)
        {
            super(key,
                  title,
                  defaultValue,
                  R.id.hl_sto_switch_title,
                  R.id.hl_sto_switch_summary,
				  R.id.hl_sto_switch_switch,
				  null);
        }

        @Override
		public SwitchSettingsObject build()
		{
			return new SwitchSettingsObject(this);
		}
	}
}
