package io.github.cutelights.sdk;

import java.util.List;

public class Main {
    
    public static void main(String[] args) {
        List<Light> lights = LightDiscoverer.discover();

        for (Light light : lights) {
            System.out.println("Light ID: " + light.getId());
            System.out.println("Light Name: " + light.getName());
            System.out.println("Light is on: " + light.getIsOn());
            System.out.println("Light brightness: " + light.getBrightness());
            System.out.println("Light color: " + light.getRed() + ", " + light.getGreen() + ", " + light.getBlue());
            System.out.println();
        }
    }
}