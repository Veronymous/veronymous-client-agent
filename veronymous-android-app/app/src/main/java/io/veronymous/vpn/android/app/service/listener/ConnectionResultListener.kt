package io.veronymous.vpn.android.app.service.listener

interface ConnectionResultListener {

    fun onSuccess()

    fun onFailure(e: Exception?)
}