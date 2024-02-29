package com.imuxuan.floatingview;

/**
 * Created by liyunpeng on 17/11/29.
 */
public interface MagnetViewListener {

    void onMagnetViewRemoved(FloatingMagnetView magnetView);

    void onMagnetViewClicked(FloatingMagnetView magnetView);

    void onMangetViewMoved(FloatingMagnetView magnetView, float x, float y);
}
