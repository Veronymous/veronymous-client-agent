package io.veronymous.client.jni;

public class VeronymousClientJni {

    public static native String newServersState();

    public static native GetServersResult getServers(String serversState);

    public static native String newClientState();

    public static native ConnectResult connect(String domain, String clientState, String serversState);

    public static native AuthenticateResult authenticate(String username, String password, String clientState);
}
