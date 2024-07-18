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

 class Frame {
    private static native long nativeNew();
    
    private static native void nativeSetColor(long framePointer, long lightPointer, int red, int green, int blue);
    private static native void nativeSetBrightness(long framePointer, long lightPointer, int brightness);
    private static native void nativeSetOn(long framePointer, long lightPointer, boolean on);
    
    private static native void nativeRun(long framePointer);
    private static native void nativeClear(long framePointer);
    private static native void nativeFree(long framePointer);

    private long framePointer;

    public Frame() {
        framePointer = nativeNew();
    }

    public void setColor(Light light, int red, int green, int blue) {
        nativeSetColor(framePointer, light.getPointer(), red, green, blue);
    }

    public void setBrightness(Light light, int brightness) {
        nativeSetBrightness(framePointer, light.getPointer(), brightness);
    }

    public void setOn(Light light, boolean on) {
        nativeSetOn(framePointer, light.getPointer(), on);
    }

    public void setBrightnessAll(Light[] lights, int brightness) {
        for (Light light : lights) {
            setBrightness(light, brightness);
        }
    }

    public void setOnAll(Light[] lights, boolean on) {
        for (Light light : lights) {
            setOn(light, on);
        }
    }

    public void setColorAll(Light[] lights, int red, int green, int blue) {
        for (Light light : lights) {
            setColor(light, red, green, blue);
        }
    }

    public void run() {
        nativeRun(framePointer);
    }

    public void clear() {
        nativeClear(framePointer);
    }

    public void free() {
        nativeFree(framePointer);
    }

    public long getPointer() {
        return framePointer;
    }
}
