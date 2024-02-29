package me.underworld.dante.views;

import android.graphics.Color;
import android.util.ArrayMap;
import android.view.LayoutInflater;
import android.view.View;
import android.widget.FrameLayout;
import android.widget.ImageView;
import android.widget.TextView;

import me.underworld.dante.R;
import me.underworld.dante.helpers.LangDict;

public class TransparentView extends DanteViewGroup {

    public View root = null;
    public ArrayMap<String, View> views = new ArrayMap<>();
    public TransparentClickListener transparentClickListener = null;

    public interface TransparentClickListener {
        void TransparentItemOnClick(String tag, View view);
    }

    @Override
    public void addViews() {
        root = LayoutInflater.from(ctx).inflate(R.layout.transparent_menu, null);
        root.setLayoutParams(new FrameLayout.LayoutParams(-2, -2, 3));
        //((TextView) root.findViewById(R.id.dt_floatmenu_settings_text)).setText(LangDict.getStringValue("dt_ft_settings"));
        attachViewsToRoot(R.id.dt_floatmenu_settings_icon, R.id.dt_floatmenu_settings_text, R.drawable.ic_settings,"settings");
        attachViewsToRoot(R.id.dt_floatmenu_about_icon, R.id.dt_floatmenu_about_text, R.drawable.ic_about,"about");
//        attachViewsToRoot(R.id.dt_floatmenu_statistics_icon, R.id.dt_floatmenu_statistics_text, R.drawable.ic_statistics,"statistics");
        Views.add(root);;
    }
    public void attachViewsToRoot(int viewId, int textId, int imageId, String tag){
        View view = root.findViewById(viewId);
        if(imageId != -1){
            ((ImageView) view).setImageResource(imageId);
        }
        view.setBackgroundColor(Integer.MIN_VALUE);
        TextView textView = (TextView) root.findViewById(textId);
        textView.setShadowLayer(3,1,1, Color.parseColor("black"));
        View view2 = (View) textView.getParent();
        view2.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                TransparentView.this.onClickDispatch(tag, v);
            }
        });
        views.put(tag, view2);
    }

    public void onClickDispatch(String tag, View view){
        if(transparentClickListener != null){
            transparentClickListener.TransparentItemOnClick(tag, view);
        }
    }
}
