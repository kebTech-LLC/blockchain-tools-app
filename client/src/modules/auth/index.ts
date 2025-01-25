import { decodeCredential, type CallbackTypes } from "vue3-google-login";
import { auth } from "..";

export class Auth {
    loggedIn: boolean = false;
    jwt: string = '';

    googleCallback(response: CallbackTypes.CredentialPopupResponse) {
        console.log('googleCallback', response);
        auth.loggedIn = true;
        auth.jwt = response.credential;
        const userData = decodeCredential(response.credential);
        console.log('userData', userData);
    }
}