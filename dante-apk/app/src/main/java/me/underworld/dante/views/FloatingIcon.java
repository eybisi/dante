package me.underworld.dante.views;

import android.graphics.drawable.Drawable;
import android.util.Log;

import com.imuxuan.floatingview.EnFloatingView;

import me.underworld.dante.DanteGlobal;
import me.underworld.dante.R;

public class FloatingIcon extends DanteViewGroup {
    public EnFloatingView floating_view = null;

    @Override
    public void addViews() {
        EnFloatingView floating_icon = new EnFloatingView(this.ctx, R.drawable.devil01, this.attachedFrame.getWidth(),this.attachedFrame.getHeight());
        floating_icon.setX(40);
        floating_icon.setY(10);
        floating_icon.setTag("icon");
        this.Views.add(floating_icon);
        this.floating_view = floating_icon;
    }
}
