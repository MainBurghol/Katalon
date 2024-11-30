import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('https://preprod-ula-content-library.joacademy.tech/login')

WebUI.maximizeWindow()

WebUI.sendKeys(findTestObject('Login/Email'), 'main.burghol@joacademy.com')

WebUI.sendKeys(findTestObject('Login/Password'), 'main1234')

WebUI.click(findTestObject('Login/button_Sign in'))

WebUI.delay(5)

WebUI.click(findTestObject('ContentLibraries/Content Libraries'))

WebUI.click(findTestObject('ContentLibraries/button_Filter'))

WebUI.click(findTestObject('ContentLibraries/FilterByStatus'))

WebUI.click(findTestObject('ContentLibraries/Select_InActive'))

WebUI.click(findTestObject('ContentLibraries/SearchBar'))

WebUI.delay(5)

WebUI.verifyElementText(findTestObject('ContentLibraries/NumberOfFilters'), '1')

WebUI.verifyElementVisible(findTestObject('ContentLibraries/CheckTheInActiveLiberary'))

WebUI.closeBrowser()