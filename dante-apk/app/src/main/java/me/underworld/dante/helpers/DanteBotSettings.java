package me.underworld.dante.helpers;

public enum DanteBotSettings {
    EMULATOR_BYPASS(0),
    INFINITE_GENIE(1),
    AUTO_LOOT(2),
    IMNOTROBOT_BYPASS(3),
    MOVE_IN_ANIMATION(4),
    LUPON_HACK(5);

    public final int value;

    DanteBotSettings(int value) {
        this.value = value;
    }
}
