import { initializeApp } from "https://www.gstatic.com/firebasejs/10.6.0/firebase-app.js";
import {
  getAuth,
  signInWithPopup,
  onAuthStateChanged,
  signOut,
  GoogleAuthProvider,
} from "https://www.gstatic.com/firebasejs/10.6.0/firebase-auth.js";

const firebaseConfig = {
  apiKey: "AIzaSyDhegaVeR028wDqdIpaFEiRyneptYbMN_E",
  authDomain: "tyche-vtt.firebaseapp.com",
  projectId: "tyche-vtt",
  storageBucket: "tyche-vtt.appspot.com",
  messagingSenderId: "137252967327",
  appId: "1:137252967327:web:fb0a7df5657eb713c423aa",
};

initializeApp(firebaseConfig);

const auth = getAuth();
const signInBtn = document.getElementById("quickstart-sign-in");
const signInMsg = document.getElementById("quickstart-message");

function toggleSignIn() {
  if (!auth.currentUser) {
    const provider = new GoogleAuthProvider();

    signInWithPopup(auth, provider).catch(function (error) {
      const errorCode = error.code;

      if (errorCode === "auth/account-exists-with-different-credential") {
        alert(
          "You have already signed up with a different auth provider for that email.",
        );
      } else {
        console.error(error);
      }
    });
  } else {
    signOut(auth);
  }
  signInBtn.disabled = true;
}

function initApp() {
  onAuthStateChanged(auth, function (user) {
    if (user) {
      signInBtn.textContent = "Sign out";

      const params = new URLSearchParams(window.location.search);
      const session = params.get("session");

      fetch("http://localhost:3000/v1/" + session, {
        method: "POST",
        mode: "no-cors",
        body: user.accessToken,
      });

      signInMsg.textContent =
        "You can safely close this window now, and return to the app";
    } else {
      signInBtn.textContent = "Sign in with Google";
    }
    signInBtn.disabled = false;
  });

  signInBtn.addEventListener("click", toggleSignIn);

  if (!auth.currentUser) {
    toggleSignIn();
  }
}

window.onload = function () {
  initApp();
};
