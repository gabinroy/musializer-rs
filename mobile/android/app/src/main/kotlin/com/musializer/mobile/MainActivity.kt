package com.musializer.mobile

import android.os.Bundle
import io.flutter.embedding.android.FlutterActivity

class MainActivity : FlutterActivity() {
    companion object {
        init {
            try {
                System.loadLibrary("musializer_core")
            } catch (e: UnsatisfiedLinkError) {
                e.printStackTrace()
            }
        }
    }

    private external fun initAndroidContext(context: Any)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        try {
            initAndroidContext(applicationContext)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}
