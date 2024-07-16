package io.github.cutelights.sdk;

public class Light {
    private static native void _setOn(long ptr, boolean on);

    private static native void _setBrightness(long ptr, int brightness);

    private static native void _setColor(long ptr, int red, int green, int blue);

    private static native boolean _getIsOn(long ptr);

    private static native int _getBrightness(long ptr);

    private static native int _getRed(long ptr);

    private static native int _getGreen(long ptr);

    private static native int _getBlue(long ptr);

    private static native String _getId(long ptr);

    private static native String _getName(long ptr);

    private static native String _free(long ptr);

    static {
        System.loadLibrary("cute_lights_java_sdk");
    }

    private long ptr;

    public Light(long ptr) {
        this.ptr = ptr;
        System.out.println("Light created with ptr: " + ptr);
    }

    public void setOn(boolean on) {
        _setOn(ptr, on);
    }

    public void setBrightness(int brightness) {
        _setBrightness(ptr, brightness);
    }

    public void setColor(int red, int green, int blue) {
        _setColor(ptr, red, green, blue);
    }

    public boolean getIsOn() {
        return _getIsOn(ptr);
    }

    public int getBrightness() {
        return _getBrightness(ptr);
    }

    public int getRed() {
        return _getRed(ptr);
    }

    public int getGreen() {
        return _getGreen(ptr);
    }

    public int getBlue() {
        return _getBlue(ptr);
    }

    public String getId() {
        return _getId(ptr);
    }

    public String getName() {
        return _getName(ptr);
    }

    @Override
    public void finalize() {
        _free(ptr);
    }
}
