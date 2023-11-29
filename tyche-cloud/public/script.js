import { initializeApp } from "https://www.gstatic.com/firebasejs/10.6.0/firebase-app.js";
import { getAuth, signInWithPopup, onAuthStateChanged, signOut, GoogleAuthProvider } from "https://www.gstatic.com/firebasejs/10.6.0/firebase-auth.js";

const firebaseConfig = {
    apiKey: "AIzaSyBZ2-HmnOLcYu3UBoN7F6hJUGskbaC9MHQ",
    authDomain: "tyche-cloud.firebaseapp.com",
    projectId: "tyche-cloud",
    storageBucket: "tyche-cloud.appspot.com",
    messagingSenderId: "371213536784",
    appId: "1:371213536784:web:8a4a2ff5b80116ddf04495"
};

initializeApp(firebaseConfig);

const auth = getAuth();
const signInBtn = document.getElementById('quickstart-sign-in');

function toggleSignIn() {
    if (!auth.currentUser) {
        const provider = new GoogleAuthProvider();

        signInWithPopup(auth, provider)
            .catch(function(error) {
                const errorCode = error.code;

                if (errorCode === 'auth/account-exists-with-different-credential') {
                    alert('You have already signed up with a different auth provider for that email.');
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
    onAuthStateChanged(auth, function(user) {
        if (user) {
            signInBtn.textContent = 'Sign out';

            const params = new URLSearchParams(window.location.search);
            const session = params.get('session');

            fetch("/v1", {
                method: "POST",
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ id: session, token: user.accessToken })
            });
        } else {
            signInBtn.textContent = 'Sign in with Google';
        }
        signInBtn.disabled = false;
    });

    signInBtn.addEventListener('click', toggleSignIn);

    if (!auth.currentUser) {
        toggleSignIn()
    }
}

window.onload = function() {
    initApp();
};
