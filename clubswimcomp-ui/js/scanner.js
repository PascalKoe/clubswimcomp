export async function init_scanner(connectedId, scannedId) {
    let connectedElement = document.getElementById(connectedId);
    let scannedElement = document.getElementById(scannedId);

    let serialPort = await navigator.serial.requestPort();
    await serialPort.open({
        baudRate: 115200
    });
    connectedElement.value = "connected";
    connectedElement.dispatchEvent(new Event('input'));

    while (serialPort.readable) {
        const reader = serialPort.readable.getReader();
        const textDecoder = new TextDecoder();

        try {
            while (true) {
                const { value, done } = await reader.read();
                if (done) {
                    break;
                }
                
                scannedElement.value = textDecoder.decode(value);
                scannedElement.dispatchEvent(new Event('input'));
            }
        } catch {
            console.log("Failed to read from serial port");
        } finally {
            reader.releaseLock();
        }
    }

    connectedElement.value = "disconnected";
    connectedElement.dispatchEvent(new Event('input'));
}
