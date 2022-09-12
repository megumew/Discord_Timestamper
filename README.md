# Discord_Timestamper
A timestamp generator for Discord using the command line.

This application uses the current time to find a timestamp based on the input options of how much time in the future or past you wish the timestamp to occur at.

**The application currently does not support inputting a specific Date/Time but this may be supported in the future.**

Timestamps appear in the form <t:```Time in seconds since Epoch```:```format character```> and example of this is:
```
<t:1662941664:F>
```
This would print off the Long Date/Time format when sent in the Discord client. 

More information on Discord timestamps can be found on [Discords Developer Docs](https://discord.com/developers/docs/reference#message-formatting-timestamp-styles).

## Usage 
After cloning and building this repository, you can run this program by calling the program or the help function for more information: 
```
discord_ts [OPTIONS]

discord_ts --help
```
