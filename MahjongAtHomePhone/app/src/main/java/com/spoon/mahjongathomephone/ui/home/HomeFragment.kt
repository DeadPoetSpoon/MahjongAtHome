package com.spoon.mahjongathomephone.ui.home

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.core.widget.doAfterTextChanged
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.spoon.mahjongathomephone.App
import com.spoon.mahjongathomephone.databinding.FragmentHomeBinding


class HomeFragment : Fragment() {

    private var _binding: FragmentHomeBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val homeViewModel =
            ViewModelProvider(this)[HomeViewModel::class.java]

        _binding = FragmentHomeBinding.inflate(inflater, container, false)
        val root: View = binding.root

        _binding?.editTextTextEmailAddress?.doAfterTextChanged { text -> homeViewModel.userSignupInfo.email = text.toString() }
        _binding?.editTextTextPassword?.doAfterTextChanged { text -> homeViewModel.userSignupInfo.psd = text.toString() }
        _binding?.editTextServerText?.doAfterTextChanged { text -> App.getInstance().setBaseUrl(text.toString()) }
        _binding?.buttonSignup?.setOnClickListener { signupClick(homeViewModel) }

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
    fun signupClick(homeViewModel: HomeViewModel) {
        val msg = "email: ${homeViewModel.userSignupInfo.email} \n psd: ${homeViewModel.userSignupInfo.psd}"
        Toast.makeText(this.context, msg, Toast.LENGTH_SHORT).show()
        homeViewModel.signup()
    }

}