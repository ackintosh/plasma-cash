pragma solidity >=0.5.0 <0.6.0;

contract RootChain {
    /*
     * Storage
     */
    uint public depositCount;

    /*
     * Events
     */
    event Deposit(address depositor, uint256 amount, uint256 uid);

    constructor()
    public
    {
        depositCount = 0;
    }

    function deposit(
        uint amount
    )
    payable
    public
    returns (uint)
    {
        // ETH
        require(amount * 10**18 == msg.value);

        uint uid = uint256(keccak256(abi.encodePacked(msg.sender, depositCount)));

        depositCount += 1;

        emit Deposit(msg.sender, amount, uid);
        return uid;
    }
}
