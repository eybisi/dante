package me.underworld.dante.views;

import android.app.Activity;
import android.content.res.Resources;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.FrameLayout;
import android.content.Context;
import java.util.ArrayList;
import java.util.List;

import me.underworld.dante.DanteGlobal;


public abstract class DanteViewGroup {
    public static List<DanteViewGroup> danteViewGroups = new ArrayList<>();
    public static boolean s0 = false;
    public Activity attachedActivity = null;
    public Context ctx;
    public FrameLayout attachedFrame = null;
    public List<View> Views = new ArrayList<>();
    public boolean viewsAdded = false;
    public boolean visible = false;
    public boolean haveViews = false;

    public Resources resources;

    public void init(Activity activity, FrameLayout frame) {
        this.attachedActivity = activity;
        this.ctx = DanteGlobal.ctx;
        this.attachedFrame = frame;
        this.viewsAdded = false;
        this.resources = DanteGlobal.resources;
        addViews();
        if(this.Views.size() == 0 ){
            boolean z = this.haveViews;
        }
        danteViewGroups.add(this);

    }

    public static ViewGroup inflateWithCtx(int layout, Context ctx) {
        try{
            return (ViewGroup) LayoutInflater.from(ctx).inflate(layout, null);
        }catch (Exception e){
            return null;
        }
    }

    public ViewGroup inflateWithoutCtx(int layout) {
        try{
            return (ViewGroup) LayoutInflater.from(this.ctx).inflate(layout, null);
        }catch (Exception e){
            return null;
        }
    }

    public static void b(boolean z ){
        if (s0 == z) {
            return;
        }
        s0 = z;
        for(DanteViewGroup danteViewGroup : danteViewGroups) {
            if(danteViewGroup.viewsAdded && danteViewGroup.haveViews) {
                if (z) {
                    for(View view : danteViewGroup.Views) {
                        view.setVisibility(View.GONE);
                    }
                } else {
                    danteViewGroup.setViewVisible();
                }
            }
        }
    }

    public void addViewsToWindow() {
        for (View view : this.Views) {
            if (!view.isAttachedToWindow()) {
                this.attachedFrame.addView(view);
            }
        }
        this.viewsAdded = true;
    }

    public void setViewVisible() {
        this.visible = true;
        if (s0) {
            return;
        }
        for (View view : this.Views) {
            view.setVisibility(View.VISIBLE);
        }
    }
    public void setViewGone() {
        for (View view : this.Views) {
            view.setVisibility(View.GONE);
        }
        this.visible = false;
    }

    public void setView(boolean visible) {
        if (visible) {
            setViewVisible();
        } else {
            setViewGone();
        }
    }


    public void changeState() {
        if (this.visible) {
            setViewGone();
        } else {
            setViewVisible();
        }
    }

    public void setX(float x) {
        for (View view : this.Views) {
            view.setX(x);
        }
    }

    public void setY(float y) {
        for (View view : this.Views) {
            view.setY(y);
        }
    }

    public float getMinX() {
        float f2 = 9999.0f;
        for (View view : this.Views) {
            if (f2 > view.getX()) {
                f2 = view.getX();
            }
        }
        return f2;
    }

    public float getMinY() {
        float f2 = 9999.0f;
        for (View view : this.Views) {
            if (f2 > view.getY()) {
                f2 = view.getY();
            }
        }
        return f2;
    }

    public abstract void addViews();
    
}
