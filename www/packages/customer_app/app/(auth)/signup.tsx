import { signupFinishRegistrationMutation } from "@/__generated__/signupFinishRegistrationMutation.graphql";
import { signupStartRegistrationMutation } from "@/__generated__/signupStartRegistrationMutation.graphql";
import { useRedirectIfAuthenticated } from "@/hooks/useRedirectIfAuthenticated";
import { handleGqlError } from "@/lib/error_gql";
import { addSeconds } from "@/lib/time";
import { toastErr } from "@/lib/toast";
import { router } from "expo-router";
import React from "react";
import {
  KeyboardAvoidingView,
  Platform,
  Text,
  TextInput,
  TouchableOpacity,
} from "react-native";
import { graphql, useMutation } from "react-relay";

export default function SignUp() {
  useRedirectIfAuthenticated();

  const [startRegistration, isStartRegistrationInFlight] =
    useMutation<signupStartRegistrationMutation>(graphql`
      mutation signupStartRegistrationMutation(
        $input: UserAccountStartRegistrationInput!
      ) {
        userAccountStartRegistration(input: $input) {
          case {
            __typename
            ... on StartRegistrationCaseAccountExist {
              foo
            }
            ... on StartRegistrationCaseOtpCode {
              digits
              ttlSeconds
              e164PhoneNumberStr
            }
          }
        }
      }
    `);

  const [finishRegistration, isFinishRegistrationInFlight] =
    useMutation<signupFinishRegistrationMutation>(graphql`
      mutation signupFinishRegistrationMutation(
        $input: UserAccountFinishRegistrationInput!
      ) {
        userAccountFinishRegistration(input: $input) {
          session {
            id
          }
        }
      }
    `);

  const [nextAllowedOtpSent, setNextAllowedOtpSent] = React.useState<Date>();
  const [phoneNumber, setPhoneNumber] = React.useState("");
  const [otpCode, setOtpCode] = React.useState("");
  const [password, setPassword] = React.useState("");

  const handleStartRegistration = React.useCallback(() => {
    if (isStartRegistrationInFlight) {
      return;
    }
    if (
      nextAllowedOtpSent !== undefined &&
      nextAllowedOtpSent.getTime() > Date.now()
    ) {
      toastErr("Please wait until time elapsed");
    }
    startRegistration({
      variables: {
        input: {
          accountType: "CUSTOMER",
          phoneNumber,
        },
      },
      onCompleted: (data) => {
        const responseCase = data.userAccountStartRegistration.case;
        switch (responseCase.__typename) {
          case "StartRegistrationCaseAccountExist": {
            toastErr(
              "Account already exist. TODO: focus to phone number input",
            );
            break;
          }
          case "StartRegistrationCaseOtpCode": {
            setNextAllowedOtpSent(
              addSeconds(new Date(), responseCase.ttlSeconds),
            );
          }
          default: {
            toastErr("Toast unexpected variant");
            break;
          }
        }
      },
      onError: handleGqlError,
    });
  }, [
    isStartRegistrationInFlight,
    nextAllowedOtpSent,
    phoneNumber,
    startRegistration,
  ]);

  const handleFinishRegistration = React.useCallback(() => {
    if (isFinishRegistrationInFlight) {
      return;
    }
    finishRegistration({
      variables: {
        input: {
          accountType: "CUSTOMER",
          otpCode,
          password,
          phoneNumber,
        },
      },
      onCompleted: (data) => {
        router.push("/(tabs)/home");
      },
      onError: handleGqlError,
    });
  }, [
    finishRegistration,
    isFinishRegistrationInFlight,
    otpCode,
    password,
    phoneNumber,
  ]);

  return (
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

      {/* TODO: remove required password, allow login with OTP */}
      <TextInput
        placeholder="Mật khẩu"
        secureTextEntry
        autoCapitalize="none"
        autoCorrect={false}
        textContentType="password"
        value={password}
        onChangeText={setPassword}
      />
      <TextInput
        placeholder="Mã OTP"
        keyboardType="number-pad"
        value={otpCode}
        onChangeText={setOtpCode}
      />
      {/* TODO: OTP resend count down */}

      <TouchableOpacity onPress={handleStartRegistration}>
        <Text>Gởi OTP</Text>
      </TouchableOpacity>

      <TouchableOpacity onPress={handleFinishRegistration}>
        <Text>Đăng ký</Text>
      </TouchableOpacity>

      {/*
       * TODO:
       * 1. Login with OTP code
       */}
    </KeyboardAvoidingView>
  );
}
