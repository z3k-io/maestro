#include <Encoder.h>

const int NUM_ENCODERS = 4;
const int DEFAULT_VOLUME = 20;   // 200 ~= 20%
const int MAX_VOLUME = 100;      // Maximum volume value
const int VOLUME_INCREMENT = 1;  // Volume change per tick

const int firstEncoderPin = 2;  // First encoder pin (assumes sequential pins)
const int firstButtonPin = 14;  // First button pin (assumes sequential pins)

Encoder* encoders[NUM_ENCODERS];
int digitalEncoderValues[NUM_ENCODERS];

bool* muteStates;
bool* buttonStates;
unsigned long* lastDebounceTimes;

const unsigned long debounceDelay = 175;

void setup() {
  Serial.begin(9600);

  muteStates = new bool[NUM_ENCODERS];
  buttonStates = new bool[NUM_ENCODERS];
  lastDebounceTimes = new unsigned long[NUM_ENCODERS];

  for (int i = 0; i < NUM_ENCODERS; i++) {
    int encoderPinA = firstEncoderPin + i * 2;
    int encoderPinB = firstEncoderPin + i * 2 + 1;
    int buttonPin = firstButtonPin + i;

    pinMode(buttonPin, INPUT_PULLUP);
    encoders[i] = new Encoder(encoderPinB, encoderPinA);  // Swap order to change dial direction
    digitalEncoderValues[i] = DEFAULT_VOLUME;
    encoders[i] -> write(DEFAULT_VOLUME / VOLUME_INCREMENT);

    muteStates[i] = false;
    buttonStates[i] = false;
    lastDebounceTimes[i] = 0;
  }
}

void loop() {
  updateEncoderStates();
  updateMuteStates();

  sendVolumeStates();
}

void updateEncoderStates() {
  for (int i = 0; i < NUM_ENCODERS; i++) {
    long value = encoders[i]->read();

    if (value < 0) {
      value = 0;
      encoders[i]->write(0);
    }

    if (value > MAX_VOLUME * VOLUME_INCREMENT) {
      value = MAX_VOLUME * VOLUME_INCREMENT;
      encoders[i]->write(MAX_VOLUME * VOLUME_INCREMENT);
    }

    digitalEncoderValues[i] = value / VOLUME_INCREMENT;
  }
}

void updateMuteStates() {
  unsigned long currentMillis = millis();

  for (int i = 0; i < NUM_ENCODERS; i++) {
    int buttonPin = firstButtonPin + i;
    if (digitalRead(buttonPin) == LOW) {
      if (!buttonStates[i] && (currentMillis - lastDebounceTimes[i]) > debounceDelay) {
        muteStates[i] = !muteStates[i];
        lastDebounceTimes[i] = currentMillis;
        buttonStates[i] = true;
      }
    } else {
      buttonStates[i] = false;
    }
  }
}

void sendVolumeStates() {
  String serializedData = "";

  for (int i = 0; i < NUM_ENCODERS; i++) {
    int volume = digitalEncoderValues[i];
    bool isMuted = muteStates[i];

    // if muted, make volume negative
    if (isMuted) {
      volume = volume * -1;
    }

    serializedData += String(volume);
    if (i < NUM_ENCODERS - 1) {
      serializedData += "|";
    }
  }

    Serial.println(serializedData);
    Serial.flush();
}
