package io.veronymous.android.veronymous.client.listener;


import io.veronymous.client.exceptions.VeronymousClientException;

public interface VeronymousTaskListener<T> {

    void onResult(T result);

    void onError(VeronymousClientException e);

}
