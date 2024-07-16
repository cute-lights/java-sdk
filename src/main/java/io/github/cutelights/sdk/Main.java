package io.github.cutelights.sdk;


public class Main {
    static {
        System.loadLibrary("cute_lights_java_sdk");
    }

    public static void main(String[] args) {

        Light[] lights = LightDiscoverer.discover();
        Frame frame = new Frame();

        for (Light light : lights) {
            System.out.println("Light ID: " + light.getId());
            System.out.println("Light Name: " + light.getName());
            System.out.println("Light is on: " + light.getIsOn());
            System.out.println("Light brightness: " + light.getBrightness());
            System.out.println("Light color: " + light.getRed() + ", " + light.getGreen()
                    + ", " + light.getBlue());
            System.out.println();
        }

        boolean state = true;

        while (true) {
            frame.setOnAll(lights, state);
            frame.run();
            state = !state;
            frame.clear();
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }
}