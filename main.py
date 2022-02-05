from email import message
import discord
import discord.ext.commands
import datetime
import re
import os


# import asyncio


client = discord.Client()


@client.event
async def on_ready():
    if os.path.exists(f"./guilds"):
        pass
    else:
        os.mkdir(f"./guilds")
    print("connected to discord!")


@client.event
async def on_guild_join(guild):
    if os.path.exists(f"./guilds/{guild.id}"):
        pass
    else:
        os.mkdir(f"./guilds/{guild.id}")
        os.mkdir(f"./guilds/{guild.id}/all")



async def ban(member : discord.Member, reason, ban_message):
    if len(reason) > 512:
        reason = reason[0:511]
    await member.send(ban_message)
    await member.ban(reason = reason, delete_message_days=1)


async def unban(member : discord.Member, reason):
    await member.unban(reason = reason)


@client.event
async def on_message(message):
    if message.author.bot:
        pass

    elif re.search("http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|)+\.([a-zA-Z]+)", message.content) and re.search("@everyone", message.content) and "Moderator" not in [userRoles.name for userRoles in message.author.roles]:
        with open(f"./guilds/{message.guild.id}/flagged_messages.txt", "a") as file:
            file.write(f"TimeStamp: {datetime.datetime.now().strftime('%d-%m-%Y %H:%M:%S')} Author: {message.author}, Channel ID: {message.channel.id} ({message.channel.name}), Message: {message.content}\n")

        with open(f"./guilds/{message.guild.id}/all/{message.author.id}.txt", "a") as file:
            file.write(f"TimeStamp: {datetime.datetime.now().strftime('%d-%m-%Y %H:%M:%S')} Author: {message.author}, Channel ID: {message.channel.id} ({message.channel.name}), Message: {message.content}\n")  

        await ban(message.author, f"Bot Trap - Message Content: {message.content}", "You have been kicked from the server for bot like behavior. You can rejoin through the invite link in Code Bullets description")
        await unban(message.author, "Bot trap")

    else:
        print(4)
        with open(f"./guilds/{message.guild.id}/all/{message.author.id}.txt", "a") as file:
            file.write(f"TimeStamp: {datetime.datetime.now().strftime('%d-%m-%Y %H:%M:%S')} Author: {message.author}, Channel ID: {message.channel.id} ({message.channel.name}), Message: {message.content}\n")
            
client.run("")

