package com.spoon.mahjongathomephone.ui.home

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.google.gson.annotations.SerializedName
import com.spoon.mahjongathomephone.App
import kotlinx.coroutines.launch


data class UserSignupInfo (
    @SerializedName("email")
    var email: String = "",
    @SerializedName("psd")
    var psd: String = ""
){
    constructor() : this("","")
}

class HomeViewModel : ViewModel() {

    var isSignupSuccess: Boolean = false
    var userSignupInfo: UserSignupInfo = UserSignupInfo()

    fun signup() {
        val app = App.getInstance()
        viewModelScope.launch {
            try {
                val response =  app.userService?.signup(userSignupInfo)
                val isSuccess = response?.isSuccessful
                isSignupSuccess = isSuccess == true
            }catch (e: Exception) {
                Log.e("MAtHApi","Network error: ${e.message}")
            }

        }
    }
}