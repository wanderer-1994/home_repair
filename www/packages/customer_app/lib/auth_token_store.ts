import * as SecureStore from "expo-secure-store";

const AUTH_TOKEN_KEY = "auth_token";

class AuthTokenStore {
  private static instance: AuthTokenStore;
  private inMemoryAccessToken: string | null = null;
  private constructor() {}

  /**
   * Returns the single instance of the AuthManager.
   */
  public static getInstance(): AuthTokenStore {
    if (!AuthTokenStore.instance) {
      AuthTokenStore.instance = new AuthTokenStore();
    }
    return AuthTokenStore.instance;
  }

  /**
   * Retrieve access token for api fetching
   */
  public async getAccessToken() {
    if (this.inMemoryAccessToken !== null) {
      return this.inMemoryAccessToken;
    }
    try {
      const token = await SecureStore.getItemAsync(AUTH_TOKEN_KEY);
      this.inMemoryAccessToken = token;
      return token;
    } catch (error) {
      console.error("Failed to retrieve token from SecureStore:", error);
      return null;
    }
  }

  /**
   * Set token whenever session updated
   */
  public async setAccessToken(token: string) {
    this.inMemoryAccessToken = token;
    await SecureStore.setItemAsync(AUTH_TOKEN_KEY, token);
  }

  /**
   * Clear token when sign out
   */
  public async clearAccessToken(token: string) {
    this.inMemoryAccessToken = null;
    await SecureStore.deleteItemAsync(AUTH_TOKEN_KEY);
  }
}

export const authTokenStore = AuthTokenStore.getInstance();
