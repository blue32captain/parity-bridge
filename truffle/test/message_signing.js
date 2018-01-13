var MessageSigning = artifacts.require("MessageSigningTest");

contract("MessageSigning", function() {
  it("should recover address from signed message", function() {
    var signature = "0xb585c41f3cceb2ff9b5c033f2edbefe93415bde365489c989bad8cef3b18e38148a13e100608a29735d709fe708926d37adcecfffb32b1d598727028a16df5db1b";
    var message = "0xdeadbeaf";
    var account = "0x006e27b6a72e1f34c626762f3c4761547aff1421";

    return MessageSigning.new().then(function(instance) {
      return instance.recoverAddressFromSignedMessage.call(signature, message)
    }).then(function(result) {
      assert.equal(account, result);
    })
  })

  it("should recover address from long signed message", function() {
    var signature = "0x3c9158597e22fa43fcc6636399c560441808e1d8496de0108e401a2ad71022b15d1191cf3c96e06759601c8e00ce7f03f350c12b19d0a8ba3ab3c07a71063f2b1c";
    var message = "0x111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    var account = "0x006e27b6a72e1f34c626762f3c4761547aff1421";

    return MessageSigning.new().then(function(instance) {
      return instance.recoverAddressFromSignedMessage.call(signature, message)
    }).then(function(result) {
      assert.equal(account, result);
    })
  })
})
