//! The audio player for Quran Island.
//!
//! Planned design (milestone M1): the player runs on its own thread and is
//! controlled only through messages. The app sends `Command`s (play, pause,
//! seek...) over a channel, and the player sends `Event`s back (ayah started,
//! progress, finished). Nothing else touches the audio device directly.
//!
//! Empty for now: this is built together in a later exercise.
