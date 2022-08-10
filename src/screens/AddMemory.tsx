import React, {useState} from 'react';
import {ScrollView, StyleSheet, Button} from 'react-native';
import {Appbar, Divider, Portal, Text, TextInput} from 'react-native-paper';

// import SignMessageButton from '../components/SignMessageButton';

export default function MainScreen() {
  const [memoText, setMemoText] = useState('');
  return (
    <>
      <Appbar.Header elevated mode="center-aligned">
        <Appbar.Content title="Dejavu" />
      </Appbar.Header>
      <Portal.Host>
        <ScrollView contentContainerStyle={styles.container}>
          <Text variant="bodyLarge">
            Write a message to record on the blockchain.
          </Text>
          <Divider style={styles.spacer} />
          <TextInput
            label="Title"
            onChangeText={text => {
              setMemoText(text);
            }}
            style={styles.textInput}
            value={memoText}
          />
          <Divider style={styles.spacer} />
          <Divider style={styles.spacer} />
          <Button onPress={() => {}}>Sign Message</Button>
        </ScrollView>
      </Portal.Host>
    </>
  );
}

const styles = StyleSheet.create({
  container: {
    padding: 16,
  },
  shell: {
    height: '100%',
  },
  spacer: {
    marginVertical: 16,
    width: '100%',
  },
  textInput: {
    width: '100%',
  },
});
