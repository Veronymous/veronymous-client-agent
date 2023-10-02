package io.veronymous.client.jni;

import io.veronymous.client.exceptions.IllegalStateException;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.nio.file.Paths;

public class VeronymousClientJniTest {
//
//    @BeforeAll
//    public static void setup() {
//        // TODO: Load from library directory
//        // TODO: Load in VeronymousClient
//        System.load(Paths.get("../../target/debug/libveronymous_client_jni.so").toAbsolutePath().toString());
//    }
//
//    @Test
//    public void getServers() throws IllegalStateException {
//        String serversState = VeronymousClientJni.newServersState();
//
//        GetServersResult result = VeronymousClientJni.getServers(serversState);
//
//        Assertions.assertNotNull(result);
//        Assertions.assertNotNull(result.getServers());
//        Assertions.assertNotEquals(0, result.getServers().length);
//
//        Assertions.assertTrue(result.getServersStateResult().hasUpdate());
//
//        serversState = result.getServersStateResult().getServersState();
//
//
//        result = VeronymousClientJni.getServers(serversState);
//
//        Assertions.assertNotNull(result);
//
//
////        Assertions.assertTrue(result.hasUpdate());
////
////        serversState = result.getUpdate();
////
////        System.out.println(serversState);
////
////        result = VeronymousClientJni.getServers(serversState);
////
////        Assertions.assertFalse(result.hasUpdate());
//    }
//
//    @Test
//    public void newClientState() {
//        String clientState = VeronymousClientJni.newClientState();
//
//        Assertions.assertNotNull(clientState);
//    }
//
//    @Test
//    public void connect() {
//        String clientState = VeronymousClientJni.newClientState();
//        String serversState = VeronymousClientJni.newServersState();
//
//        ConnectResult result = VeronymousClientJni.connect("ca_tor", clientState, serversState);
//
//        Assertions.assertNotNull(result);
//    }
//
//    @Test
//    public void authenticate() {
//        String clientState = VeronymousClientJni.newClientState();
//
//        AuthenticateResult result = VeronymousClientJni.authenticate("user1", "password1", clientState);
////        AuthenticateResult result = VeronymousClientJni.authenticate("user2", "password", clientState);
//
//        Assertions.assertNotNull(result);
//    }
}
