package io.veronymous.android.vpn.app.service.listener

interface ConnectionResultListener {

    fun onSuccess()

    fun onFailure(e: Exception?)
}