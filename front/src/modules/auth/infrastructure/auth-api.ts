import type { RegisterFormValues } from "../domain/schema";

export async function registerUser(data: RegisterFormValues): Promise<void> {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/signup`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email: data.email,
      password: data.password,
    }),
  });

  if (!response.ok) {
    throw new Error("Error registering user");
  }
}
