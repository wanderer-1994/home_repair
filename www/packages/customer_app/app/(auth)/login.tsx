import useSessionCheck from "@/hooks/useSessionCheck";
import { Link, useRouter } from "expo-router";
import React from "react";
import { KeyboardAvoidingView, Platform, Text, TextInput, TouchableOpacity, View } from "react-native";

export default function Login() {
  const router = useRouter();
  const check = useSessionCheck();

  React.useCallback(() => {
    if (check._status === "AUTHENTICATED") {
      router.replace("/(tabs)/home");
    }
  }, [check._status, router]);

  const [phoneNumer, setPhoneNumber] = React.useState("");
  const [password, setPassword] = React.useState("");

  const handleLogin = React.useCallback(() => {
    console.log({phoneNumer, password})
  }, [password, phoneNumer])

  return (
    <View>
      {/* KeyboardAvoidingView prevents the keyboard from hiding the input fields */}
    <KeyboardAvoidingView 
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <Text>Welcome Back</Text>

      <TextInput
        placeholder="Số điện thoại"
        keyboardType="phone-pad"
        value={phoneNumer}
        onChangeText={setPhoneNumber}
      />
      <TextInput
        placeholder="Mật khẩu"
        secureTextEntry
        autoCapitalize="none"
        autoCorrect={false}
        textContentType="password"
        value={password}
        onChangeText={setPassword}
      />

      <TouchableOpacity onPress={handleLogin}>
        <Text>LOG IN</Text>
      </TouchableOpacity>

      <Link href="/(auth)/signup">Tạo tài khoản</Link>

      {/*
        * TODO:
        * 1. Login with OTP code
        */}
    </KeyboardAvoidingView>
    </View>
  );
}
