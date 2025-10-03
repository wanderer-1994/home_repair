export function newClientError(message: string) {
  return {
    __clientError: true,
    message,
  };
}
