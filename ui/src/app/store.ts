import { authSlice } from "@/features/auth/authSlice";
import { configureStore } from "@reduxjs/toolkit";
import { persistReducer } from "redux-persist";
import logger from "redux-logger";
import storage from "./storage";

const authPersistConfig = {
  key: "auth",
  storage: storage,
  whitelist: ["token", "isAuthenticated"],
};

const rootReducer = {
  auth: persistReducer(authPersistConfig, authSlice.reducer),
};

export const makeStore = () =>
  configureStore({
    reducer: rootReducer,
    middleware: (getDefaultMiddleware) =>
      getDefaultMiddleware({
        serializableCheck: false,
      }).concat(logger),
  });

export type AppStore = ReturnType<typeof makeStore>;
export type AppDispatch = AppStore["dispatch"];
export type RootState = ReturnType<AppStore["getState"]>;
