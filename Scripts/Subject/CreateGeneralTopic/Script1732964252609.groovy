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

int min = 1000 // Minimum value

int max = 9999 // Maximum value

// Generate the random number
int randomNumber = new Random().nextInt((max - min) + 1) + min

WebUI.openBrowser('https://preprod-ula-content-library.joacademy.tech/login')

WebUI.maximizeWindow()

WebUI.sendKeys(findTestObject('Login/Email'), 'main.burghol@joacademy.com')

WebUI.sendKeys(findTestObject('Login/Password'), 'main1234')

WebUI.click(findTestObject('Login/button_Sign in'))

WebUI.delay(5)

WebUI.click(findTestObject('ContentLibraries/Content Libraries'))

WebUI.click(findTestObject('ContentLibraries/New content library'))

WebUI.delay(3)

WebUI.sendKeys(findTestObject('ContentLibraries/ContentLibraryCreation/EnglishTitle'), 'AutomationTest')

WebUI.sendKeys(findTestObject('ContentLibraries/ContentLibraryCreation/code'), randomNumber.toString())

WebUI.click(findTestObject('ContentLibraries/ContentLibraryCreation/ChangeTheLanguageInsideCreateContent'))

WebUI.click(findTestObject('ContentLibraries/ContentLibraryCreation/ChangeLangaugeToArabicContent'))

WebUI.sendKeys(findTestObject('ContentLibraries/ContentLibraryCreation/ArabicTitle'), 'فحص الاوتوميشن')

WebUI.click(findTestObject('ContentLibraries/ContentLibraryCreation/CreateButton'))

WebUI.verifyElementVisible(findTestObject('ContentLibraries/ContentLibraryCreation/Created_PopUp'))

WebUI.delay(3)

WebUI.click(findTestObject('Subject/New subject'))

WebUI.sendKeys(findTestObject('Subject/SubjectEn'), 'Subject1')

WebUI.sendKeys(findTestObject('Subject/SubjectAr'), 'مادة 1')

WebUI.click(findTestObject('Subject/Academic Subject'))

WebUI.click(findTestObject('Subject/SelectOneOfTheSubjects'))

WebUI.click(findTestObject('Subject/CreateSubject'))

WebUI.delay(5)

TestObject targetElement = findTestObject('Subject/FirstSubject')

// Scroll to the target element
WebUI.scrollToElement(targetElement, 5)

WebUI.click(findTestObject('Subject/FirstSubject'))

WebUI.click(findTestObject('Subject/New general topic'))

WebUI.sendKeys(findTestObject('Subject/GeneralTopicName'), 'dfsdfsaf')

WebUI.click(findTestObject('Subject/CreateGeneralTopic'))

WebUI.click(findTestObject('Subject/SelectGeneralTopic'))

WebUI.delay(5)

WebUI.click(findTestObject('Subject/Add to special topics'))

WebUI.sendKeys(findTestObject('Subject/SpecialTopicName'), 'gfdgsghh')

WebUI.sendKeys(findTestObject('Subject/Learning Object'), 'fadsffa')

WebUI.click(findTestObject('Subject/Save changes'))

WebUI.delay(5)

WebUI.closeBrowser()