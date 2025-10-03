package com.spoon.mahjongathomephone

import android.app.Application
import com.google.gson.Gson
import com.google.gson.GsonBuilder
import com.google.gson.Strictness
import com.spoon.mahjongathomephone.ui.home.UserSignupInfo
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import retrofit2.http.Body
import retrofit2.http.POST

import okhttp3.OkHttpClient
import okhttp3.ResponseBody
import java.security.cert.X509Certificate
import javax.net.ssl.*

fun createUnsafeOkHttpClient(): OkHttpClient {
    val trustAllCerts = arrayOf<TrustManager>(object : X509TrustManager {
        override fun checkClientTrusted(chain: Array<out X509Certificate>?, authType: String?) {}
        override fun checkServerTrusted(chain: Array<out X509Certificate>?, authType: String?) {}
        override fun getAcceptedIssuers(): Array<X509Certificate> = arrayOf()
    })

    val sslContext = SSLContext.getInstance("SSL")
    sslContext.init(null, trustAllCerts, java.security.SecureRandom())

    return OkHttpClient.Builder()
        .sslSocketFactory(sslContext.socketFactory, trustAllCerts[0] as X509TrustManager)
        .hostnameVerifier { _, _ -> true } // 跳过主机名验证
        .build()
}

interface UserService {
    @POST("/user/signup")
    suspend fun signup(@Body userSignupInfo: UserSignupInfo): Response<ResponseBody>
    @POST("/user/login")
    suspend fun login(@Body userSignupInfo: UserSignupInfo): Response<ResponseBody>
}

class App: Application() {
    companion object {
        private lateinit var instance: App

        fun getInstance(): App {
            return instance
        }
    }
    // private
    private var userToken: String = ""
    private var isLoggedIn: Boolean = false
    private var currentUserId: Int = 0
    private var baseUrl: String = ""
    private val gson: Gson = GsonBuilder()
        .setStrictness(Strictness.LENIENT)
        .create()
    private lateinit var retrofit: Retrofit

    // public
    var userService: UserService? = null

    override fun onCreate() {
        super.onCreate()
        instance = this

    }

    fun setBaseUrl(url: String) {
        try {
            baseUrl = url

            val unsafeClient = createUnsafeOkHttpClient()

            retrofit = Retrofit.Builder()
                .baseUrl(this.baseUrl)
                .client(unsafeClient)
                .addConverterFactory(GsonConverterFactory.create(gson))
                .build()

            userService = retrofit.create(UserService::class.java)
        }catch (_: Exception){

        }

    }


    fun login(user:Int,token: String){
        userToken = token
        isLoggedIn = true
        currentUserId = user
    }
}