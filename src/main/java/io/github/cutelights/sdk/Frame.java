package io.github.cutelights.sdk;

public class Frame {
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
