package io.veronymous.android.veronymous.client.model;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.List;

import io.veronymous.android.veronymous.client.exceptions.VeronymousParseException;

public class WireguardConnection {

    private static final String CLIENT_ADDRESSES = "client_addresses";
    private static final String SERVER_PUBLIC_KEY = "wg_public_key";
    private static final String SERVER_ENDPOINT = "wg_endpoint";
    private static final String CLIENT_PRIVATE_KEY = "client_private_key";
    private static final String CLIENT_PUBLIC_KEY = "client_public_key";


    private final List<String> clientAddresses;
    private final String serverPublicKey;
    private final String serverEndpoint;
    private final String clientPrivateKey;
    private final String clientPublicKey;

    public WireguardConnection(List<String> clientAddresses,
                               String serverPublicKey,
                               String serverEndpoint,
                               String clientPrivateKey,
                               String clientPublicKey) {
        this.clientAddresses = clientAddresses;
        this.serverPublicKey = serverPublicKey;
        this.serverEndpoint = serverEndpoint;
        this.clientPrivateKey = clientPrivateKey;
        this.clientPublicKey = clientPublicKey;
    }

    public List<String> getClientAddresses() {
        return clientAddresses;
    }

    public String getServerPublicKey() {
        return serverPublicKey;
    }

    public String getServerEndpoint() {
        return serverEndpoint;
    }

    public String getClientPrivateKey() {
        return clientPrivateKey;
    }

    public String getClientPublicKey() {
        return clientPublicKey;
    }

    public static WireguardConnection fromJson(String json) throws VeronymousParseException {
        try {
            JSONObject object = new JSONObject(json);

            JSONArray clientAddressesArray = object.getJSONArray(CLIENT_ADDRESSES);
            List<String> clientAddresses = new ArrayList<>(clientAddressesArray.length());

            for (int i = 0; i < clientAddressesArray.length(); i++) {
                clientAddresses.add(clientAddressesArray.getString(i));
            }

            return new WireguardConnection(
                    clientAddresses,
                    object.getString(SERVER_PUBLIC_KEY),
                    object.getString(SERVER_ENDPOINT),
                    object.getString(CLIENT_PRIVATE_KEY),
                    object.getString(CLIENT_PUBLIC_KEY)
            );

        } catch (JSONException e) {
            throw new VeronymousParseException("Could not parse wireguard connection.", e);
        }
    }
}
