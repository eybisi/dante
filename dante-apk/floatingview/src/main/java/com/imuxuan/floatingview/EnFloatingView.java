package com.imuxuan.floatingview;

import android.content.Context;
import android.graphics.drawable.Drawable;
import android.util.Log;
import android.view.LayoutInflater;
import android.widget.FrameLayout;
import android.widget.ImageView;

import me.underworld.dante.R;

public class EnFloatingView extends FloatingMagnetView {

    private final ImageView mIcon;

    public EnFloatingView( Context context,int drawable, int screenWidth,int screenHeight) {
        this(context,drawable,screenWidth,screenHeight, R.layout.en_floating_view);
    }

    public EnFloatingView(Context context, int drawable, int screenWidth, int screenHeight, int resource) {
        super(context, null);
        setScreenSize(screenWidth, screenHeight);
        LayoutInflater.from(context).inflate(resource, this, true);
        mIcon = findViewById(R.id.floating_icon);
        mIcon.setImageResource(drawable);
        mIcon.measure(0, 0);
        setIconSize(60, 60);

        setLayoutParams(new LayoutParams(-2, -2));
    }

    public void setIconImage(int resId){
        mIcon.setImageResource(resId);
    }

}
