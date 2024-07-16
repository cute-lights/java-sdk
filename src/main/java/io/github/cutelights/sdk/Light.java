package io.github.cutelights.sdk;

public class Light {

    static {
        System.loadLibrary("cute_lights_java_sdk");
    }

    public static native boolean nativeSetOn(long lightPointer, boolean on);

    public static native boolean nativeSetBrightness(long lightPointer, int brightness);

    public static native boolean nativeSetColor(long lightPointer, int red, int green, int blue);

    public static native String nativeGetName(long lightPointer);

    public static native boolean nativeGetIsOn(long lightPointer);

    public static native int nativeGetBrightness(long lightPointer);

    public static native boolean nativeGetSupportedColor(long lightPointer);

    public static native int nativeGetRed(long lightPointer);

    public static native int nativeGetGreen(long lightPointer);

    public static native int nativeGetBlue(long lightPointer);

    public static native String nativeGetId(long lightPointer);

    public static native void nativeFree(long lightPointer);

    private long lightPointer;

    public long getPointer() {
        return lightPointer;
    }

    public Light(long lightPointer) {
        this.lightPointer = lightPointer;
    }

    public boolean setOn(boolean on) {
        return nativeSetOn(lightPointer, on);
    }

    public boolean setBrightness(int brightness) {
        return nativeSetBrightness(lightPointer, brightness);
    }

    public boolean setColor(int red, int green, int blue) {
        return nativeSetColor(lightPointer, red, green, blue);
    }

    public String getName() {
        return nativeGetName(lightPointer);
    }

    public boolean getIsOn() {
        return nativeGetIsOn(lightPointer);
    }

    public int getBrightness() {
        return nativeGetBrightness(lightPointer);
    }

    public boolean getSupportedColor() {
        return nativeGetSupportedColor(lightPointer);
    }

    public int getRed() {
        return nativeGetRed(lightPointer);
    }

    public int getGreen() {
        return nativeGetGreen(lightPointer);
    }

    public int getBlue() {
        return nativeGetBlue(lightPointer);
    }

    public String getId() {
        return nativeGetId(lightPointer);
    }

    public void free() {
        nativeFree(lightPointer);
    }
}
