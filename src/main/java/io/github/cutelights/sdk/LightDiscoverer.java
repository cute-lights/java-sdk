package io.github.cutelights.sdk;

import java.util.ArrayList;
import java.util.List;

public class LightDiscoverer {
    private static native long nativeNew();

    private static native void nativeFree(long ptr);

    private static native java.lang.Long nativeNew(java.lang.Long ptr);

    private static native java.lang.Integer nativeLength(long ptr);


    static {
        System.loadLibrary("cute_lights_java_sdk");
    }
    public static List<Light> discover() {
        
        long ptr = nativeNew();
        System.out.println("ptr: " + ptr);
        System.out.println("length: " + nativeLength(ptr));
        List<Light> lights = new ArrayList<>();

        for (int i = 0; i < nativeLength(ptr); i++) {
            long lightPtr = nativeNew(ptr);
            lights.add(new Light(lightPtr));
        }

        nativeFree(ptr);
        return lights;
    }
}
