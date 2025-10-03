import { loginWithPasswordMutation } from "@/__generated__/loginWithPasswordMutation.graphql";
import { useRedirectIfAuthenticated } from "@/hooks/useRedirectIfAuthenticated";
import { handleGqlError } from "@/lib/error_gql";
import { Link, useRouter } from "expo-router";
import React from "react";
import {
  KeyboardAvoidingView,
  Platform,
  Text,
  TextInput,
  TouchableOpacity,
} from "react-native";
import { graphql, useMutation } from "react-relay";

export default function Login() {
  const router = useRouter();
  useRedirectIfAuthenticated();

  const [loginWithPassword, isInFlight] =
    useMutation<loginWithPasswordMutation>(graphql`
      mutation loginWithPasswordMutation($input: UserSignInWithPasswordInput!) {
        userSignInWithPassword(input: $input) {
          session {
            id
            actorType {
              __typename
              ... on Customer {
                id
                phoneNumber
                profile {
                  id
                  nickName
                }
              }
            }
          }
        }
      }
    `);

  const [phoneNumber, setPhoneNumber] = React.useState("");
  const [password, setPassword] = React.useState("");

  const handleLogin = React.useCallback(() => {
    if (isInFlight) {
      return;
    }
    loginWithPassword({
      variables: {
        input: {
          accountType: "CUSTOMER",
          password,
          phoneNumber,
        },
      },
      onCompleted: (data) => {
        router.replace("/(tabs)/home");
      },
      onError: handleGqlError,
    });
  }, [isInFlight, loginWithPassword, password, phoneNumber, router]);

  return (
    // KeyboardAvoidingView prevents the keyboard from hiding the input fields
    <KeyboardAvoidingView
      behavior={Platform.OS === "ios" ? "padding" : "height"}
    >
      <Text>Welcome Back</Text>

      <TextInput
        placeholder="Số điện thoại"
        keyboardType="phone-pad"
        value={phoneNumber}
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
  );
}
