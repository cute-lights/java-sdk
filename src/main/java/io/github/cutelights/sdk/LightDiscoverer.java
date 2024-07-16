package io.github.cutelights.sdk;

import java.util.ArrayList;
import java.util.List;

public class LightDiscoverer {
    public static native long[] nativeDiscover();

    static {
        System.loadLibrary("cute_lights_java_sdk");
    }


    public static Light[] discover() {
        long[] lights = nativeDiscover();
        List<Light> lightList = new ArrayList<>();
        for (long light : lights) {
            lightList.add(new Light(light));
        }
        return lightList.toArray(new Light[0]);
    }
}
